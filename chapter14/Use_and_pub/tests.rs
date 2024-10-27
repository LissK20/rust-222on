#[test]
fn test1()
{
    use std::fmt::Result as FmtResult;
    use std::io::Result as IoResult;

    fn main()
    {
        let fmt_result: FmtResult<()> = Ok(());
        let io_result: IoResult<()> = Ok(());

        println!("Both results are successfully created!");
    }
}

fn test2()
{
    use std::collections::{HashMap, BTreeMap, HashSet};

    fn main()
    {
        let _c1: HashMap<&str, i32> = HashMap::new();
        let mut c2 = BTreeMap::new();
        c2.insert(1, "a");
        let _c3: HashSet<i32> = HashSet::new();
    }

}
