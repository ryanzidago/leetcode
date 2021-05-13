fn main() {
    println!("Hello, world!");
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut previous_node: Option<Box<ListNode>> = None;
    let mut current_node: Option<Box<ListNode>> = head;
    while let Some(mut boxed_node) = current_node {
        let next = boxed_node.next.take();
        boxed_node.next = previous_node;
        previous_node = Some(boxed_node);
        current_node = next;
    }

    previous_node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_list_test() {
        let head: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        let expected: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }));

        let reversed = reverse_list(head);
        assert_eq!(reversed, expected);
    }
}
