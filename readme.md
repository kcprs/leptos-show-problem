To reproduce unexpected behaviour of `Show`:

1. Run `trunk serve --open`
2. Click on "Item 1" -> item 1 gets highlighted
3. Click on "close" of "Item 1" -> item 1 gets closed
4. Click on "Item 2" -> item 2 gets highlighted
5. Click on "close" of "Item 2" -> item 2 gets closed, `None` is unwrapped in the `Show` component that renders the text at the very top
