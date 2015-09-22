// Barely Modified from the Original C# code. None of the code has been modified to be idiomatic
// Rust, but rather the most direct translation possible that still compiles.

extern crate gilded_rose;

use gilded_rose::goblin::Item;

fn main() {
    let mut items = vec!
    {
        Item { name: "+5 Dexterity Vest", sell_in: 10, quality: 20 },
        Item { name: "Aged Brie", sell_in: 2, quality: 0 },
        Item { name: "Elixir of the Mongoose", sell_in: 5, quality: 7 },
        Item { name: "Sulfuras, Hand of Ragnaros", sell_in: 0, quality: 80 },
        Item { name: "Backstage passes to a TAFKAL80ETC concert", sell_in: 15, quality: 20 },
        Item { name: "Conjured Mana Cake", sell_in: 3, quality: 6 }
    };

    for i in 0..50 {
        println!("Day {}:\n========================================", i);
        for item in &items {
            println!("{:?}", item);
        }
        UpdateQuality(&mut items[..]);
    }
}

fn UpdateQuality(items: &mut [Item])
{
    for i in 0..items.len() 
    {
        if (items[i].name != "Aged Brie" && items[i].name != "Backstage passes to a TAFKAL80ETC concert")
        {
            if (items[i].quality > 0)
            {
                if (items[i].name != "Sulfuras, Hand of Ragnaros")
                {
                    items[i].quality = items[i].quality - 1;
                }
            }
        }
        else
        {
            if (items[i].quality < 50)
            {
                items[i].quality = items[i].quality + 1;

                if (items[i].name == "Backstage passes to a TAFKAL80ETC concert")
                {
                    if (items[i].sell_in < 11)
                    {
                        if (items[i].quality < 50)
                        {
                            items[i].quality = items[i].quality + 1;
                        }
                    }

                    if (items[i].sell_in < 6)
                    {
                        if (items[i].quality < 50)
                        {
                            items[i].quality = items[i].quality + 1;
                        }
                    }
                }
            }
        }

        if (items[i].name != "Sulfuras, Hand of Ragnaros")
        {
            items[i].sell_in = items[i].sell_in - 1;
        }

        if (items[i].sell_in < 0)
        {
            if (items[i].name != "Aged Brie")
            {
                if (items[i].name != "Backstage passes to a TAFKAL80ETC concert")
                {
                    if (items[i].quality > 0)
                    {
                        if (items[i].name != "Sulfuras, Hand of Ragnaros")
                        {
                            items[i].quality = items[i].quality - 1;
                        }
                    }
                }
                else
                {
                    items[i].quality = items[i].quality - items[i].quality;
                }
            }
            else
            {
                if (items[i].quality < 50)
{
                    items[i].quality = items[i].quality + 1;
                }
            }
        }
    }
}

#[test]
fn normal_items_decrease_quality() {
    let mut items = vec![
        Item { name: "+5 Dexterity Vest", sell_in: 10, quality: 20 },
    ];
    UpdateQuality(&mut items[..]);
    assert_eq!(items[0].sell_in, 9);
    assert_eq!(items[0].quality, 19);
}
