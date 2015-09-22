# The Gilded Rose Code Kata

This is a Rust version of the Gilded Rose Kata, found
[here](http://iamnotmyself.com/2011/02/13/refactor-this-the-gilded-rose-kata/).
Much of this README is inspired by [Jim Weirich's Ruby
Translation](https://github.com/jimweirich/gilded_rose_kata).

This is a refactorying kata, so you will be starting with a legacy code base.
There is a complex set of existing business rules that need to be extended.
You need to do so carefully, as not to break the existing behavior.

This version of the Gilded Rose is a direct translation of the original C#
code. The fewest modifications possible were made to get it compiling. It is
not idomatic Rust. Part of refactoring is making the code read more clearly, so
feel free to make the code more Rustic.

## Changes from the original

This Rust version follows the original code very closely, but has the following
changes:

* The original had no tests. Since this is a refactoring kata, I feel the tests
  are important. I have included a single test to push you in the right
  direction, but adding tests to legacy (untested) code is half of refactoring.

* The original had no visible output. It did not have any behavior to actually
  verify against. In my experience, legacy systems are always working, even if
  mysteriously. To emulate that, I've added a printout of each day's items.

* The C# code used a static List of the inventory items. Accessing mutable
  statics in Rust are considered `unsafe`, so they are very difficult to work
  with. As a result, `UpdateQuality` has been changed to take the items as an
  argument.

* The Item data structure is not to be changed. It is a public api that we do
  not control. I've moved the Item struct into a separate module to emphasize
  this boundary.


# The Gilded Rose

Hi and welcome to team Gilded Rose. As you know, we are a small inn with a
prime location in a prominent city run by a friendly innkeeper named Allison.
We also buy and sell only the finest goods. Unfortunately, our goods are
constantly degrading in quality as they approach their sell by date. We have a
system in place that updates our inventory for us. It was developed by a
no-nonsense type named Leeroy, who has moved on to new adventures. Your task is
to add the new feature to our system so that we can begin selling a new
category of items. First an introduction to our system:

- All items have a SellIn value which denotes the number of days we have to
  sell the item
- All items have a Quality value which denotes how valuable the item is
- At the end of each day our system lowers both values for every item

Pretty simple, right? Well this is where it gets interesting:

  - Once the sell by date has passed, Quality degrades twice as fast
  - The Quality of an item is never negative
  - "Aged Brie" actually increases in Quality the older it gets
  - The Quality of an item is never more than 50
  - "Sulfuras", being a legendary item, never has to be sold or decreases in
    Quality
  - "Backstage passes", like aged brie, increases in Quality as it's SellIn
    value approaches; Quality increases by 2 when there are 10 days or less and
    by 3 when there are 5 days or less but Quality drops to 0 after the concert

We have recently signed a supplier of conjured items. This requires an update
to our system:

- "Conjured" items degrade in Quality twice as fast as normal items

Feel free to make any changes to the UpdateQuality method and add any new code
as long as everything still works correctly. However, do not add or remove
fields in the Item struct as those belong to the goblin in the corner who will
insta-rage and one-shot you as he doesn't believe in shared code ownership.

# Some Guidance

If you aren't sure where to start, try:

* Reading the original inventory rules again. Pull up the code and try to spot
  where each rule is currently being enforced. Resist the temptation to change
  anything yet.

* Adding some new tests. This will require you to understand the existing
  features, and give you some assurances that you won't break the current
  behavior as you refactor the code.

* Examining the original output of the program before making any changes. Look
  at the original_behavior.log. You can diff this against the output midway.
  This should add additional assurance that your tests are catching everything.
  When you spot a regression, add a new test so it doesn't happen again.

* Try to remove any compiler warnings or lints. These are indications of
  problems existing within the code. These are often quick wins for improving
  the code.

* Extract existing behavior into smaller functions. Break up unrelated
  behavior. Group common functionality together. Give these new functions good
  names. Reevaluate the current names you are using. You should be able to
  write a few tests for these smaller functions more easily, as the functions
  will have fewer responsibilities.

* Add the new rules for "Conjured" items. Afterall, this is what we originally
  set out to do.

