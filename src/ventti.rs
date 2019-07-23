
pub struct Game {
    max: u32,
}

#[derive(Clone)]
struct Node {
    pub parent: Option<Box<Node>>,
    pub children: Vec<Node>,
    pub bet: Bet,
}

#[derive(Clone)]
struct Bet {
    pub amount: u32,
    pub total: u32,
    pub iteration: u8,
}

impl Node {
    pub fn new_root() -> Node {
         Node {
            parent: None,
            children: Vec::new(),
            bet: Bet::first(),
        }
    }

    fn bet_history(&self) -> Vec<u32> {
        let mut bets = match &self.parent {
            Some(p) => { p.bet_history() },
            None => { Vec::new() }
        };
        bets.push(self.bet.amount);
        bets
    }

    fn count_children(&self) -> u32 {
        let mut count = 1;
        for child in &self.children {
            count += child.count_children();
        };
        count
    }

    fn highest_iteration(&self) -> Vec<u8> {
        let mut iterations = Vec::new();
        iterations.push(self.bet.iteration);
        for child in &self.children {
            iterations.append(&mut child.highest_iteration());
        };
        iterations
    }

    fn print(&self) {
        println!("{}\t{}\t{:?}", self.bet.amount, self.bet.total, self.bet_history());
    }
}

impl Bet {
    pub fn first() -> Bet {
        Bet {
            amount: 50,
            total: 50,
            iteration: 1,
        }
    }
}

impl Game {
    pub fn new(max: u32) -> Game {
        Game {
            max: max,
        }
    }

    pub fn find(&self) {
        let tree = self.build_tree(Node::new_root());
        let highest_iteration = *tree.highest_iteration().iter().max().unwrap();
        println!("Last\tNeeded\tBets for max {}, {} iterations", self.max, highest_iteration);
        println!("All count: {}", tree.count_children());
        let all = Game::get_tree(tree, highest_iteration);
        all.first().unwrap().print();
        all.last().unwrap().print();
    }

    fn get_tree(node: Node, max_iteration: u8) -> Vec<Node> {
        if node.bet.iteration >= max_iteration &&
            node.children.len() == 0 {
            [node].to_vec()
        } else {
            let mut ret = Vec::new();
            for child in node.children {
                ret.append(&mut Game::get_tree(child, max_iteration));
            }
            ret
        }
    }

    fn build_tree(&self, node: Node) -> Node {
        let mut children = Vec::new();
        for amount in (node.bet.total..(node.bet.total*3)).step_by(50) {
            let total = node.bet.total + amount;
            if amount <= self.max {
                let new_node = Node {
                    parent: Some(Box::new(node.clone())),
                    children: Vec::new(),
                    bet: Bet {
                        amount: amount,
                        total: total,
                        iteration: node.bet.iteration + 1,
                    },
                };
                children.push(self.build_tree(new_node));
            } else {
                break;
            }
        }
        Node {
            parent: node.parent,
            children: children,
            bet: node.bet,
        }
    }
}
