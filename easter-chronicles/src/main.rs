use chrono::{NaiveDate};

fn calculate_easter_gauss(year: i32) -> NaiveDate {
    let (m, n) = if year < 1900 { (23, 4) } else { (24, 5) };
    let a = year % 19;
    let b = year % 4;
    let c = year % 7;
    let d = (19 * a + m) % 30;
    let e = (2 * b + 4 * c + 6 * d + n) % 7;
    let easter_day = 22 + d + e;

    if easter_day > 31 {
        NaiveDate::from_ymd_opt(year, 4, (easter_day - 31) as u32).unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, 3, easter_day as u32).unwrap()
    }
}

fn tell_easter_story(year: i32) {
    println!("\n🎭 THE EASTER CHRONICLES FOR YEAR {} 🎭", year);
    println!("{}", "=".repeat(50));
    
    let easter = calculate_easter_gauss(year);
    println!("🧮 Gaussian Result: {}", easter.format("%A, %B %d, %Y"));
    
    match year {
        1818 => {
            println!("\n🎯 MATHEMATICAL BOUNDARY:");
            println!("   This is the EARLIEST possible Easter in Gregorian calendar!");
            println!("   March 22 - when lunar and solar cycles perfectly align");
        },
        1964 => {
            let good_friday = easter - chrono::Duration::days(2);
            println!("\n🌍 EARTH-SHAKING HISTORY:");
            println!("   Good Friday {}: Great Alaska Earthquake (9.2 magnitude)", 
                     good_friday.format("%B %d"));
            println!("   The earth trembled during Holy Week, yet Easter arrived on schedule!");
        },
        2038 => {
            println!("\n⏰ TEMPORAL EXTREME:");
            println!("   This is the LATEST possible Easter in Gregorian calendar!");
            println!("   April 25 - the mathematical maximum boundary");
        },
        _ => {
            println!("\n📖 Mathematical Poetry: Each date emerges from cosmic harmony");
        }
    }
    
    // Show Gaussian breakdown
    let (m, n) = if year < 1900 { (23, 4) } else { (24, 5) };
    let a = year % 19;
    let d = (19 * a + m) % 30;
    println!("\n🔢 Gaussian Variables: a={} d={} → Easter emerges from pure arithmetic", a, d);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        // User specified a year
        if let Ok(year) = args[1].parse::<i32>() {
            tell_easter_story(year);
        } else {
            println!("❌ Invalid year: {}", args[1]);
        }
    } else {
        // Default: show all three boundary dates
        println!("🎭 Easter Chronicles - Historical Boundary Tests 🎭");
        println!("{}", "=".repeat(60));
        
        // Test 1818 - Earliest possible Easter
        let easter_1818 = calculate_easter_gauss(1818);
        println!("1️⃣  1818 Easter: {} ⬅️ EARLIEST POSSIBLE", easter_1818.format("%B %d, %Y"));
        
        // Test 1964 - Good Friday Earthquake year  
        let easter_1964 = calculate_easter_gauss(1964);
        let good_friday_1964 = easter_1964 - chrono::Duration::days(2);
        println!("2️⃣  1964 Easter: {} 🌍 Earthquake on Good Friday: {}", 
                 easter_1964.format("%B %d, %Y"),
                 good_friday_1964.format("%B %d"));
        
        // Test 2038 - Latest possible Easter
        let easter_2038 = calculate_easter_gauss(2038);
        println!("3️⃣  2038 Easter: {} ➡️ LATEST POSSIBLE", easter_2038.format("%B %d, %Y"));
        
        println!("\n🧮 Gauss's algorithm captures the complete mathematical space!");
        println!("💡 Try: cargo run -- YEAR  (e.g., cargo run -- 1964)");
    }
}
