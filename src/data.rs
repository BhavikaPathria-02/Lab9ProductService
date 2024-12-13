use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Iphone 14 Pro Max".to_string(),
            price: 120.99,
            description: "The iPhone 14 Pro Max pushes the boundaries of innovation with its advanced A-series chip, delivering unmatched speed and efficiency. It features a cutting-edge ProMotion display for smooth visuals and a dynamic triple-camera system designed to capture stunning photos and videos in any light. Wrapped in a sleek, robust design, this phone offers an immersive user experience with improved battery life and enhanced privacy features.".to_string(),
            image: "/BestBuy-Iphone.png".to_string()
        },
        Product {
            id: 2,
            name: "Wireless Mouse".to_string(),
            price: 26.99,
            description: "The wireless mouse provides a clutter-free workspace by eliminating the need for cables, offering freedom and flexibility with its robust wireless connectivity. It features an ergonomic design for comfortable use during extended periods, and adjustable DPI settings for precision tracking. Equipped with long-lasting batteries and compatible with multiple operating systems, it’s ideal for both office and personal use.".to_string(),
            image: "/WirelessMouse-BestBuy.png".to_string()
        },
        Product {
            id: 3,
            name: "Dell Laptop".to_string(),
            price: 559.99,
            description: "The Dell Laptop combines powerful performance with high-end design, featuring the latest Intel processors and crisp, vibrant displays for an optimal viewing experience. It comes equipped with a durable battery, ample storage space, and advanced connectivity options to cater to both business professionals and casual users. Lightweight and elegant, this laptop offers reliability and ease of use, making it a top choice for everyday computing tasks.".to_string(),
            image: "/BestBuy-LaptopDell.png".to_string()
        }
       
    ]
}