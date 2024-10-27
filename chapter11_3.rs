#[test]
fn test1()
{
    use std::collections::HashMap;

    fn main()
    {
        let mut scores: HashMap<&str, i32> = HashMap::new();
        scores.insert("Sunface", 98);
        scores.insert("Daniel", 95);
        scores.insert("Ashley", 69);
        scores.insert("Katie", 58);

        let score = scores.get("Sunface");
        assert_eq!(score, Some(&98));

        if scores.contains_key("Daniel")
        {
            let score = scores["Daniel"];
            assert_eq!(score, 95);
            scores.remove("Daniel");
        }

        assert_eq!(scores.len(), 3);

        for (name, score) in &scores
        {
            println!("The score of {} is {}", name, score);
        }
    }
}

fn test2()
{
    use std::collections::HashMap;
    fn main()
    {
        let teams = [
            ("Chinese Team", 100),
            ("American Team", 10),
            ("France Team", 50),
        ];

        let mut teams_map1 = HashMap::new();
        for team in &teams
        {
            teams_map1.insert(team.0, team.1);
        }

        let mut teams_map2 = HashMap::new();
        for team in &teams
        {
            teams_map2.insert(team.0, team.1);
        }

        let teams_map3: HashMap<_, _> = teams.iter().cloned().collect();

        assert_eq!(teams_map1, teams_map2);
        assert_eq!(teams_map1, teams_map3);
        println!("Success!");
    }
}

fn test3()
{
    use std::collections::HashMap;
    fn main()
    {
        let mut player_stats = HashMap::new();

        player_stats.entry("health").or_insert(100);

        assert_eq!(player_stats["health"], 100);

        player_stats.entry("health").or_insert_with(random_stat_buff);
        assert_eq!(player_stats["health"], 100);

        let health = player_stats.entry("health").or_insert(50);
        assert_eq!(health, &100);
        *health -= 50;
        assert_eq!(*health, 50);

        println!("Success!");
    }
    fn random_stat_buff() -> u8
    {
        42
    }
}

fn test4()
{
    use std::collections::HashMap;
    #[derive(Debug)]
    struct Viking
    {
        name: String,
        country: String,
    }

    impl Viking
    {
        fn new(name: &str, country: &str) -> Viking
        {
            Viking
            {
                name: name.to_string(),
                country: country.to_string(),
            }
        }
    }

    fn main()
    {
        let vikings: HashMap<Viking, u32> = HashMap::from([
            (Viking::new("Einar", "Norway"), 25),
            (Viking::new("Olaf", "Denmark"), 24),
            (Viking::new("Harald", "Iceland"), 12),
        ]);

        for (viking, health) in &vikings
        {
            println!("{:?} has {} hp", viking, health);
        }
    }
}

fn test5()
{
    use std::collections::HashMap;
    fn main()
    {
        let v1 = 10;
        let mut m1 = HashMap::new();
        m1.insert(v1, v1);
        println!("v1 is still usable after inserting to hashmap : {}", v1);

        let v2 = "hello".to_string();
        let mut m2 = HashMap::new();
        m2.insert(v2.clone(), v1);

        assert_eq!(v2, "hello");
        println!("Success!");
    }
}
