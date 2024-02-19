use crate::{dice, dice::LengthRange, Die, Fate};

pub trait TreeBuilder<T> {
    fn branch_count(&self, fate: Fate, max_branch_count: usize) -> usize;
    fn build(&mut self, fate: Fate, subtrees: impl ExactSizeIterator<Item = T>) -> T;
}

pub fn tree<T, B>(builder_die: impl Die<B>, node_count_range: impl LengthRange) -> impl Die<T>
where
    B: TreeBuilder<T>,
{
    let node_count_die = dice::length(node_count_range);

    dice::from_fn(move |mut fate| {
        let mut builder = fate.roll(&builder_die);
        let node_count = fate.roll(&node_count_die);
        let root_idx = 0;

        // Generate tree structure
        let nodes = {
            let mut nodes: Vec<Vec<usize>> = Vec::with_capacity(node_count.saturating_add(1));
            let mut nodes_to_intialize: Vec<(usize, usize)> = Vec::new();

            // Add root
            nodes.push(Vec::new());
            nodes_to_intialize.push((root_idx, node_count));

            // Initialize the nodes from top to button
            while let Some((parent_idx, remaining_size_of_parent)) = nodes_to_intialize.pop() {
                // Create children for parent
                if remaining_size_of_parent > 0 {
                    // Branch randomly
                    let branch_count = builder.branch_count(fate.copy(), remaining_size_of_parent);
                    assert!(
                        branch_count >= 1,
                        "Branch count is {}, but must be >= 1",
                        branch_count
                    );
                    assert!(
                        branch_count <= remaining_size_of_parent,
                        "Branch count is {}, but must be <= {}",
                        branch_count,
                        remaining_size_of_parent
                    );

                    // Split remaining size randomly
                    let remaining_size_of_children = fate.roll(dice::terms_of_usize(
                        remaining_size_of_parent - branch_count,
                        branch_count,
                    ));

                    for remaining_size_of_child in remaining_size_of_children {
                        let child_idx = nodes.len();

                        // Add child
                        nodes.push(Vec::new());
                        nodes_to_intialize.push((child_idx, remaining_size_of_child));

                        // Register child to parant
                        let children = &mut nodes[parent_idx];
                        children.push(child_idx);
                    }
                }
            }

            nodes
        };

        // Generate actual tree
        let tree = {
            // Create slots for storing the subtrees temporarily
            let mut subtree_slots: Vec<Option<T>> = Vec::with_capacity(nodes.len());
            for _ in 0..nodes.len() {
                subtree_slots.push(None)
            }

            // Create the subtrees from buttom to top
            for (idx, children) in nodes.into_iter().enumerate().rev() {
                // Collect subtrees
                let mut subtrees = Vec::with_capacity(children.len());
                for idx in children {
                    subtrees.push(subtree_slots[idx].take().unwrap());
                }

                // Merge subtrees
                let expression = builder.build(fate.copy(), subtrees.into_iter());
                subtree_slots[idx] = Some(expression);
            }

            // Return the root
            subtree_slots[root_idx].take().unwrap()
        };

        tree
    })
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::TreeBuilder;

    // Tree type for testing
    #[derive(Debug, Clone)]
    enum Expression {
        Constant(u32),
        Add(Box<Expression>, Box<Expression>),
        Sub(Box<Expression>, Box<Expression>),
        Sum(Vec<Expression>),
    }

    impl Expression {
        fn into_node_count(self) -> usize {
            let mut acc = 0;
            let mut expressions = vec![self];
            while let Some(expression) = expressions.pop() {
                acc += 1;
                match expression {
                    Expression::Constant(_) => (),
                    Expression::Add(left, right) => expressions.extend([*left, *right]),
                    Expression::Sub(left, right) => expressions.extend([*left, *right]),
                    Expression::Sum(subtrees) => expressions.extend(subtrees),
                }
            }
            // Subtract 1 because the root doesn't count
            acc - 1
        }

        fn into_max_depth(self) -> usize {
            let mut acc = 0;
            let mut expressions = vec![(0, self)];
            while let Some((max_depth, expression)) = expressions.pop() {
                let max_depth = max_depth + 1;
                match expression {
                    Expression::Constant(_) => {
                        acc = acc.max(max_depth);
                    }
                    Expression::Add(left, right) => {
                        expressions.extend([(max_depth, *left), (max_depth, *right)])
                    }
                    Expression::Sub(left, right) => {
                        expressions.extend([(max_depth, *left), (max_depth, *right)])
                    }
                    Expression::Sum(subtrees) => {
                        if subtrees.is_empty() {
                            acc = acc.max(max_depth);
                        } else {
                            expressions.extend(subtrees.into_iter().map(|e| (max_depth, e)))
                        }
                    }
                }
            }
            // Subtract 1 because the root doesn't count
            acc - 1
        }
    }

    #[derive(Clone)]
    struct ExpressionBuilder;

    impl TreeBuilder<Expression> for ExpressionBuilder {
        fn branch_count(&self, mut fate: Fate, max_branch_count: usize) -> usize {
            fate.roll(dice::weighted_one_of_die().three(
                (16, dice::just(1)),
                (4, dice::just(2.min(max_branch_count))),
                (1, dice::uni_usize(1..=max_branch_count)),
            ))
        }

        fn build(
            &mut self,
            mut fate: Fate,
            mut subtrees: impl ExactSizeIterator<Item = Expression>,
        ) -> Expression {
            match subtrees.len() {
                0 => {
                    if fate.roll(dice::weighted_bool(1, 10)) {
                        let constant = fate.roll(dice::u32(..));
                        Expression::Constant(constant)
                    } else {
                        Expression::Sum(Vec::new())
                    }
                }
                2 => {
                    let right = Box::new(subtrees.next().unwrap());
                    let left = Box::new(subtrees.next().unwrap());
                    if fate.roll(dice::bool()) {
                        Expression::Add(left, right)
                    } else {
                        Expression::Sub(left, right)
                    }
                }
                _ => Expression::Sum(subtrees.collect()),
            }
        }
    }

    #[test]
    fn tree_has_correct_node_count() {
        Dicetest::repeatedly().run(|mut fate| {
            let node_count = fate.roll(dice::length(..));
            let tree = fate.roll(dice::tree(dice::just(ExpressionBuilder), node_count));
            assert_eq!(node_count, tree.into_node_count());
        });
    }

    #[test]
    fn tree_calc_stats() {
        Dicetest::repeatedly()
            .passes(0)
            .stats_enabled(true)
            .run(|mut fate| {
                let node_count = 100000;
                let tree = fate.roll(dice::tree(dice::just(ExpressionBuilder), node_count));
                let max_depth = tree.into_max_depth();
                let resolution = 10;
                stat!(
                    "max depth",
                    "~{}",
                    (max_depth / resolution) * resolution,
                );
            })
    }
}
