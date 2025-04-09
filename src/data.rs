use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        // Product {
        //     id: 1,
        //     name: "Contoso Catnip's Friend".to_string(),
        //     price: 9.99,
        //     description: "Watch your feline friend embark on a fishing adventure with Contoso Catnip's Friend toy. Packed with irresistible catnip and dangling fish lure.".to_string(),
        //     image: "/catnip.jpg".to_string()
        // },
        // Product {
        //     id: 2,
        //     name: "Salty Sailor's Squeaky Squid".to_string(),
        //     price: 6.99,
        //     description: "Let your dog set sail with the Salty Sailor's Squeaky Squid. This interactive toy provides hours of fun, featuring multiple squeakers and crinkle tentacles.".to_string(),
        //     image: "/squid.jpg".to_string()
        // },
        // Product {
        //     id: 3,
        //     name: "Mermaid's Mice Trio".to_string(),
        //     price: 12.99,
        //     description: "Entertain your kitty with the Mermaid's Mice Trio. These adorable plush mice are dressed as mermaids and filled with catnip to captivate their curiosity.".to_string(),
        //     image: "/mermaid.jpg".to_string()
        // },
        // Product {
        //     id: 4,
        //     name: "Ocean Explorer's Puzzle Ball".to_string(),
        //     price: 11.99,
        //     description: "Challenge your pet's problem-solving skills with the Ocean Explorer's Puzzle Ball. This interactive toy features hidden compartments and treats, providing mental stimulation and entertainment.".to_string(),
        //     image: "/ocean.jpg".to_string()
        // },
        // Product {
        //     id: 5,
        //     name: "Pirate Parrot Teaser Wand".to_string(),
        //     price: 8.99,
        //     description: "Engage your cat in a playful pursuit with the Pirate Parrot Teaser Wand. The colorful feathers and jingling bells mimic the mischievous charm of a pirate's parrot.".to_string(),
        //     image: "/pirate.jpg".to_string()
        // },
        // Product {
        //     id: 6,
        //     name: "Seafarer's Tug Rope".to_string(),
        //     price: 14.99,
        //     description: "Tug-of-war meets nautical adventure with the Seafarer's Tug Rope. Made from marine-grade rope, it's perfect for interactive play and promoting dental health in dogs.".to_string(),
        //     image: "/tug.jpg".to_string()
        // },
        // Product {
        //     id: 7,
        //     name: "Seashell Snuggle Bed".to_string(),
        //     price: 19.99,
        //     description: "Give your furry friend a cozy spot to curl up with the Seashell Snuggle Bed. Shaped like a seashell, this plush bed provides comfort and relaxation for cats and small dogs.".to_string(),
        //     image: "/bed.jpg".to_string()
        // },
        // Product {
        //     id: 8,
        //     name: "Nautical Knot Ball".to_string(),
        //     price: 7.99,
        //     description: "Unleash your dog's inner sailor with the Nautical Knot Ball. Made from sturdy ropes, it's perfect for fetching, tugging, and satisfying their chewing needs.".to_string(),
        //     image: "/knot.jpg".to_string()
        // },
        // Product {
        //     id: 9,
        //     name: "Contoso Claw's Crabby Cat Toy".to_string(),
        //     price: 3.99,
        //     description: "Watch your cat go crazy for Contoso Claw's Crabby Cat Toy. This crinkly and catnip-filled toy will awaken their hunting instincts and provide endless entertainment.".to_string(),
        //     image: "/crabby.jpg".to_string()
        // },
        // Product {
        //     id: 10,
        //     name: "Ahoy Doggy Life Jacket".to_string(),
        //     price: 5.99,
        //     description: "Ensure your furry friend stays safe during water adventures with the Ahoy Doggy Life Jacket. Designed for dogs, this flotation device offers buoyancy and visibility in style.".to_string(),
        //     image: "/lifejacket.jpg".to_string()
        // }
        Product {
            id: 1,
            name: "Dell XPS 13 Laptop".to_string(),
            price: 1299.99,
            description: "Experience premium performance with the Dell XPS 13, featuring a 13.4-inch InfinityEdge display, 11th Gen Intel Core i7 processor, and 512GB SSD.".to_string(),
            image: "/xps13.png".to_string()
        },
        Product {
            id: 2,
            name: "Apple MacBook Pro 14\"".to_string(),
            price: 1999.99,
            description: "Power through workflows with the Apple MacBook Pro, equipped with the M2 Pro chip, Liquid Retina XDR display, and long-lasting battery life.".to_string(),
            image: "/macbookpro.png".to_string()
        },
        Product {
            id: 3,
            name: "Samsung 65\" QLED 4K TV".to_string(),
            price: 1399.99,
            description: "Transform your entertainment with Samsung's QLED technology, delivering vibrant colors, sharp contrast, and immersive 4K clarity on a 65-inch screen.".to_string(),
            image: "/samsungqled.png".to_string()
        },
        Product {
            id: 4,
            name: "Sony WH-1000XM5 Headphones".to_string(),
            price: 399.99,
            description: "Enjoy industry-leading noise cancellation and crystal-clear sound with the Sony WH-1000XM5 wireless headphones.".to_string(),
            image: "/sonyheadphones.png".to_string()
        },
        Product {
            id: 5,
            name: "iPhone 15 Pro Max".to_string(),
            price: 1499.99,
            description: "Stay ahead with Apple's iPhone 15 Pro Max featuring A17 Pro chip, ProMotion technology, and advanced camera system.".to_string(),
            image: "/iphone15.png".to_string()
        },
        Product {
            id: 6,
            name: "Logitech MX Master 3S Mouse".to_string(),
            price: 129.99,
            description: "Boost productivity with the ergonomic Logitech MX Master 3S, offering ultra-quiet clicks, MagSpeed scrolling, and multi-device support.".to_string(),
            image: "/logitechmx3.png".to_string()
        },
        Product {
            id: 7,
            name: "ASUS ROG Strix Gaming Laptop".to_string(),
            price: 1799.99,
            description: "Dominate the battlefield with the ASUS ROG Strix, powered by AMD Ryzen 9 and NVIDIA RTX 4070 for cutting-edge gaming performance.".to_string(),
            image: "/rogstrix.png".to_string()
        },
        Product {
            id: 8,
            name: "Google Nest Thermostat".to_string(),
            price: 129.99,
            description: "Save energy and control your home's climate with the Google Nest Thermostat, featuring smart scheduling and voice control.".to_string(),
            image: "/nestthermostat.png".to_string()
        },
        Product {
            id: 9,
            name: "Canon EOS R50 Mirrorless Camera".to_string(),
            price: 799.99,
            description: "Capture stunning photos and videos with the compact Canon EOS R50, equipped with a 24.2MP sensor and 4K recording.".to_string(),
            image: "/canoneosr50.png".to_string()
        },
        Product {
            id: 10,
            name: "Anker 737 Power Bank".to_string(),
            price: 109.99,
            description: "Keep your devices charged with the Anker 737 Power Bank, offering 140W fast charging and 24,000mAh capacity.".to_string(),
            image: "/ankerpower.png".to_string()
        }

    ]
}