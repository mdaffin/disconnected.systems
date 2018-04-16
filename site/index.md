---
home: true
---
<PostItem 
  v-for="post in posts"
  v-bind:title="post.title"
  v-bind:to="post.path"
  v-bind:date="post.date"
  v-bind:description="post.description"
/>

<script>
export default {
    computed: {
        posts() {
            return this.$site.pages
            .filter((page) => page.path.startsWith("/blog/"))
            .map((page) => ({
                title: page.title,
                path: page.path,
                date: new Date(page.frontmatter.date),
                description: page.frontmatter.description,
                tags: page.frontmatter.tags,
            }))
            .sort((a, b) => b.date - a.date);
        }
    }
}
</script>
