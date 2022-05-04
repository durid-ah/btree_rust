use std::cell::RefMut;

use super::Node;

fn rebalance_after_delete(node_to_rebalance: &mut RefMut<Node>, removed_key_idx: usize)
{
   let has_than_min_keys = 
      node_to_rebalance.keys.len() < node_to_rebalance.min_keys;

   if !has_than_min_keys { return; }

   if node_to_rebalance.try_move_key_from_left_child(removed_key_idx).is_ok()
   {
      return;
   }
   
   if node_to_rebalance.try_move_key_from_right_child(removed_key_idx).is_ok()
   {
      return;
   }

   // TODO: Split if full after merging the children
   //    - If it has children borrow
   //    - Figure out how to merge the leaf logic with the other
   //    stuff
}