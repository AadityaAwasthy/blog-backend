use blog::Post;


fn main() {
    let mut my_blog = Post::new();

    //Adding content to my new blog Post
    my_blog.add_text("This program intends to use OOP paradigm to implement the backend for a blog posting app, this program is a replica of the one you find in the book The Rust Programming Language");

    //Trying to get content from the post using the content method, should not return anything as
    //it is in a draft state
    println!("Content of the draft : {}",my_blog.content());

    //Changing state to pending review and again trying to access content, which it should not
    //allow as this is still not approved
    my_blog.request_review();
    println!("Content of the draft : {}",my_blog.content());

    //Approving the blog and publishing it, this changes the state to published and we should now
    //be able to access the content of the blog
    my_blog.approve();
    println!("Content of the draft : {}",my_blog.content());
}
