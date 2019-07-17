
pub struct Game {
    max: i64,
    increments: Vec<i64>,
}

#[derive(Clone)]
struct Node {
    pub parent: Option<Box<Node>>,
    pub children: Vec<Node>,
    pub bet: Bet,
}

#[derive(Clone)]
struct Bet {
    pub amount: i64,
    pub total: i64,
    pub iteration: i64,
}

impl Node {
    fn bet_history(&self) -> Vec<i64> {
        let mut bets = match &self.parent {
            Some(p) => { p.bet_history() },
            None => { Vec::new() }
        };
        bets.push(self.bet.amount);
        bets
    }

    fn highest_iteration(&self) -> Vec<i64> {
        let mut iterations = Vec::new();
        iterations.push(self.bet.iteration);
        for child in &self.children {
            iterations.append(&mut child.highest_iteration());
        };
        iterations
    }
}

impl Bet {
}

impl Game {
    pub fn new(max: i64) -> Game {
        let mut increments = Vec::new();
        for x in (0..=10_000).step_by(50) {
            increments.push(x);
        }
        Game {
            max: max,
            increments: increments,
        }
    }

    pub fn find(&self) {
        let tree = self.build_tree(Game::build_root());
        let highest_iteration = *tree.highest_iteration().iter().max().unwrap();
        println!("Done! {} Increments", highest_iteration);
        println!("Last\tNeeded");
        Game::print_tree(tree, highest_iteration);
    }

    fn print_tree(node: Node, max_iteration: i64) {
        if node.bet.iteration >= max_iteration {
            println!("{}\t{}\t{:?}", node.bet.amount, node.bet.total, node.bet_history());
        }
        for child in node.children {
            Game::print_tree(child, max_iteration);
        }
    }

    fn build_root() -> Node {
         Node {
            parent: None,
            children: Vec::new(),
            bet: Bet {
                amount: 50,
                total: 50,
                iteration: 1,
            },
        }
    }

    fn build_tree(&self, node: Node) -> Node {
        let mut children = Vec::new();
        for increment in &self.increments {
            let amount = node.bet.amount + increment;
            let total = node.bet.total + amount;
            let iteration = node.bet.iteration + 1;
            if amount > 0 && amount <= self.max && amount * 2 >= total {
                let new_node = Node {
                    parent: Some(Box::new(node.clone())),
                    children: Vec::new(),
                    bet: Bet {
                        amount: amount,
                        total: total,
                        iteration: iteration,
                    },
                };
                // println!("{}, {}, {}, {:?}", new_node.bet.amount, new_node.bet.total, new_node.bet.iteration, new_node.bet_history());
                children.push(self.build_tree(new_node));
            }
        }
        Node {
            parent: node.parent,
            children: children,
            bet: node.bet,
        }
    }
}
