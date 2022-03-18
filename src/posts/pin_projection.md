# Pin Projection

When working with pinned structs, the question arises how one can access the fields of that struct in a method that takes just `Pin<&mut Struct>`. The usual approach is to write helper methods (so called projections) that turn `Pin<&mut Struct>` into a reference to the field, but what type should that reference have? Is it `Pin<&mut Field>` or `&mut Field`? The same question arises with the fields of an enum, and also when considering container/wrapper types such as `Vec<T>`, `Box<T>`, or `RefCell<T>`. (This question applies to both mutable and shared references, we just use the more common case of mutable references here for illustration.)

当使用固定结构体（`struct`）时，就会遇到这个的问题：如何访问内部字段？
通常是引入辅助方法，把结构体引用转换为字段引用，也就是所谓的投影（Projection）。
那么返回的字段引用该是什么类型？要不要套上 `Pin` 呢？

此外，当使用枚举（`enum`）或者包装类型（譬如`Vec<T>`、`Box<T>`）时，也有同样的问题。
这个问题不只适用于可变引用（`Pin<&mut Struct>`），普通引用（`Pin<&Struct>`）同样适用。
下面仅以可变引用为例，简单阐述。

It turns out that it is actually up to the author of the data structure to decide whether the pinned projection for a particular field turns `Pin<&mut Struct>` into `Pin<&mut Field>` or `&mut Field`. There are some constraints though, and the most important constraint is consistency: every field can be either projected to a pinned reference, or have pinning removed as part of the projection. If both are done for the same field, that will likely be unsound!

字段引用要不要套上 `Pin` 呢？自己看着办。不过有限制，最重要的是要保持一致，所有字段要么都套，要不都不套。
如果某字段既返回 `Pin<&mut Field>` 又返回 `&mut Field`，就要出乱子（`unsound`）。

As the author of a data structure you get to decide for each field whether pinning "propagates" to this field or not. Pinning that propagates is also called "structural", because it follows the structure of the type. In the following subsections, we describe the considerations that have to be made for either choice.

字段套上 `Pin` 称为结构性的（`structural`），因为它跟结构体保持一致。

## Pinning is not structural for field

It may seem counter-intuitive that the field of a pinned struct might not be pinned, but that is actually the easiest choice: if a `Pin<&mut Field>` is never created, nothing can go wrong! So, if you decide that some field does not have structural pinning, all you have to ensure is that you never create a pinned reference to that field.

结构体固定而字段不固定，这有点反直觉。但这却是最简单的情况，如果不创建 `Pin<&mut Field>` 啥事也没有。
如果字段不需要固定，那就确保不要创建固定引用。

Fields without structural pinning may have a projection method that turns `Pin<&mut Struct>` into `&mut Field`:

没有结构性固定，那么投影方法如下：

```rust
impl Struct {
    fn pin_get_field(self: Pin<&mut Self>) -> &mut Field {
        // This is okay because `field` is never considered pinned.
        unsafe { &mut self.get_unchecked_mut().field }
    }
}
```

You may also `impl Unpin` for `Struct` even if the type of field is not `Unpin`. What that type thinks about pinning is not relevant when no `Pin<&mut Field>` is ever created.

## Pinning is structural for field

The other option is to decide that pinning is "structural" for field, meaning that if the struct is pinned then so is the field.

另外一种就是结构性字段，这意味着如果结构体固定，那么该字段也是固定的。

This allows writing a projection that creates a `Pin<&mut Field>`, thus witnessing that the field is pinned:

这就需要投影方法创建固定引用，`Pin<&mut Field>`。

```rust
impl Struct {
    fn pin_get_field(self: Pin<&mut Self>) -> Pin<&mut Field> {
        // This is okay because `field` is pinned when `self` is.
        unsafe { self.map_unchecked_mut(|s| &mut s.field) }
    }
}
```

However, structural pinning comes with a few extra requirements:

然而，结构性固定有一些额外条件。

The struct must only be `Unpin` if all the structural fields are `Unpin`. This is the default, but `Unpin` is a safe trait, so as the author of the struct it is your responsibility not to add something like `impl<T> Unpin for Struct<T>`. (Notice that adding a projection operation requires unsafe code, so the fact that `Unpin` is a safe trait does not break the principle that you only have to worry about any of this if you use unsafe.)

如果所有字段都是 `Unpin`，那么该结构体也必须是 `Unpin`。
这是默认情形，但 `Unpin` 是安全的特性，因此作为该结构体的作者，不添加 `impl<T> Unpin for Struct<T>` 之类的是你的责任。（注意：添加映射操作要用到 `unsafe` 代码，而 `Unpin` 作为一个安全的特性并不会破坏你所担心的原则，当你用到用到 `unsafe` 代码。）

> 操！这都说的是啥？？？

The destructor of the struct must not move structural fields out of its argument. This is the exact point that was raised in the previous section: `drop` takes `&mut self`, but the struct (and hence its fields) might have been pinned before. You have to guarantee that you do not move a field inside your `Drop` implementation. In particular, as explained previously, this means that your struct must not be `#[repr(packed)]`. See that section for how to write drop in a way that the compiler can help you not accidentally break pinning.

在解构结构体时，禁止挪出结构性字段。这也是前一节提到的：`drop` 函数入参为 `&mut self`，但该结构体，及相关字段是固定的。
你得确保 `Drop` 实现中没有移动字段。特别地，正如之前解释的，这意味着该结构体不能是 `#[repr(packed)]`。
参考相关章节，确保在 `drop` 时编译器不会破坏固定。

You must make sure that you uphold the `Drop` guarantee: once your struct is pinned, the memory that contains the content is not overwritten or deallocated without calling the content's destructors. This can be tricky, as witnessed by `VecDeque<T>`: the destructor of `VecDeque<T>` can fail to call drop on all elements if one of the destructors panics. This violates the `Drop` guarantee, because it can lead to elements being deallocated without their destructor being called. (`VecDeque<T>` has no pinning projections, so this does not cause unsoundness.)

你必须确保持有 `Drop` 资源：一旦该结构体固定住，只要不屌用析构函数，相应的内存就不会被覆盖，或释放。
这有些花巧，以 `VecDeque<T>` 为例，调用析构函数释放所有元素，只要有一个 `panics`，整个就会失败。
这就不满足 `Drop` 约定，因为析构函数没调用，元素就释放掉了。（由于 `VecDeque<T>` 没有固定投影，这不会导致 unsoundness）。

You must not offer any other operations that could lead to data being moved out of the structural fields when your type is pinned. For example, if the struct contains an `Option<T>` and there is a take-like operation with type `fn(Pin<&mut Struct<T>>) -> Option<T>`, that operation can be used to move a `T` out of a pinned `Struct<T>` -- which means pinning cannot be structural for the field holding this data.

此外，当结构体固定时，其他能移动结构性字段也不能有。比如，某个结构体有个 `Option<T>` 字段，你想整个类似 `take` 的方法，
类型为 `fn(Pin<&mut Struct<T>>) -> Option<T>`，用于把 `T` 从结构体中挪出来。这意味着该字段就不能是结构性的。

For a more complex example of moving data out of a pinned type, imagine if `RefCell<T>` had a method `fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T>`. Then we could do the following:

我们来弄个更复杂的例子。假如 `RefCell<T>` 有方法 `fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T>`，我们就可以这么做：

```rust
fn exploit_ref_cell<T>(rc: Pin<&mut RefCell<T>>) {
    { let p = rc.as_mut().get_pin_mut(); } // Here we get pinned access to the `T`.
    let rc_shr: &RefCell<T> = rc.into_ref().get_ref();
    let b = rc_shr.borrow_mut();
    let content = &mut *b; // And here we have `&mut T` to the same data.
}
```

This is catastrophic, it means we can first pin the content of the `RefCell<T>` (using `RefCell::get_pin_mut`) and then move that content using the mutable reference we got later.

这将是灾难性的，它意味着我们可以先固定 `RefCell<T>`（使用上面假设的方法，`RefCell::get_pin_mut`）然后使用可变引用来挪动内容。

## Examples

For a type like `Vec<T>`, both possibilities (structural pinning or not) make sense. A `Vec<T>` with structural pinning could have `get_pin/get_pin_mut` methods to get pinned references to elements. However, it could not allow calling pop on a pinned `Vec<T>` because that would move the (structurally pinned) contents! Nor could it allow push, which might reallocate and thus also move the contents.

像 `Vec<T>` 之类的类型，无论是解构性固定与否，都有意义。结构性固定，可以拥有 `get_pin/get_pin_mut` 方法，来获取内部元素的固定引用。然而，这又不允许调用 `pop` 方法，因为他会移动内容，也不允许 `push`，他会重新分配内存，也会移动内容。

A `Vec<T>` without structural pinning could `impl<T> Unpin for Vec<T>`, because the contents are never pinned and the `Vec<T>` itself is fine with being moved as well. At that point pinning just has no effect on the vector at all.

没有结构性固定，就可以 `impl<T> Unpin for Vec<T>`。因为内容就不会固定，`Vec<T>` 本身也可以移动。此时，固定就不起作用。

In the standard library, pointer types generally do not have structural pinning, and thus they do not offer pinning projections. This is why `Box<T>`: `Unpin` holds for all `T`. It makes sense to do this for pointer types, because moving the `Box<T>` does not actually move the `T`: the `Box<T>` can be freely movable (aka `Unpin`) even if the T is not. In fact, even `Pin<Box<T>>` and `Pin<&mut T>` are always Unpin themselves, for the same reason: their contents (the T) are pinned, but the pointers themselves can be moved without moving the pinned data. For both `Box<T>` and `Pin<Box<T>>`, whether the content is pinned is entirely independent of whether the pointer is pinned, meaning pinning is not structural.

在标准库中，指针类型一般都没有结构性固定，因此他们也没有固定投影。这也是 `Box<T>`: `Unpin` holds for all `T`。
这对指针类型是有用的，因为移动 `Box<T>` 实质上并不会移动 `T`：`Box<T>` 可以随便移动，即便 `T` 不是 `Unpin`。
事实上，连 `Pin<Box<T>>` 和 `Pin<&mut T>` 自身也是 `Unpin` 的，原因也是如此：其内容 `T` 是固定的，它指针自身却可以随便移动，而不需要移动所指向的数据。`Box<T>` 与 `Pin<Box<T>>`，其内容固定与否，跟指针固定与否毫不相关，这也意味着固定不是结构性的。

When implementing a Future combinator, you will usually need structural pinning for the nested futures, as you need to get pinned references to them to call poll. But if your combinator contains any other data that does not need to be pinned, you can make those fields not structural and hence freely access them with a mutable reference even when you just have `Pin<&mut Self>` (such as in your own poll implementation).

在实现 `Future` 结合子时，通常需要固定住内部嵌套的 `Future`，因为需要获取它们的固定引用来调用 `poll` 方法。
但如果里面包含其他不需要固定的数据，它们就是非结构性的，可以使用可变引用随便访问，即使只有 `Pin<&mut Self>`（就像自定是 `poll` 实现一样）。
