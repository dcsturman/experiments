use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut answers = HashSet::new();
    let mut a = 1;
    let mut b = 2;

    loop {
        let c = Root::compute(a * a + b * b);

        if c.int && a + b + c.val == sum {
            answers.insert([a, b, c.val]);
        }

        // If I've advanced b to the point where a + b + c is greater than the target sum,
        // I now need to try the next possible value for 'a'.
        if a + b + c.val > sum {
            // if the next value for 'a' (a+1) summed with the minimal values for 'b' and
            // 'c' together exceed sum, I've explored all possible values.
            if 3 * a + 6 > sum {
                return answers;
            }

            // Pick the next 'a' value and then the starting 'b' value with that.
            a = a + 1;
            b = a + 1;
            continue;
        }

        b = b + 1;
    }
}

struct Root {
    int: bool,
    val: u32,
}

impl Root {
    fn compute(val: u32) -> Root {
        let root = (val as f32).sqrt() as u32;
        Root {
            int: (val == root * root),
            val: root,
        }
    }
}
