package traversal

/**
 * Definition for a binary tree node.
 */
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func buildTree(preorder []int, inorder []int) *TreeNode {
	if len(preorder) == 0 || len(preorder) != len(inorder) {
		return nil
	}
	root := &TreeNode{Val: preorder[0]}

	i := 0
	for inorder[i] != root.Val {
		i++
	}

	root.Left = buildTree(preorder[1:1+i], inorder[:i])
	root.Right = buildTree(preorder[1+i:], inorder[1+i:])
	return root
}
