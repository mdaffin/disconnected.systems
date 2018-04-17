---
home: true
---
<PostItem 
  v-for="post in posts()"
  :key="post.frontmatter.date"
  v-bind:title="post.title"
  v-bind:to="post.path"
  v-bind:date="new Date(post.frontmatter.date)"
  v-bind:description="post.frontmatter.description"
/>

<script>
export default {
    methods: {
        posts_with_tag(tag) {
            return this.$site.pages
            .filter((page) => page.frontmatter.tags)
            .filter((page) => page.frontmatter.tags.includes(tag));
        },
        posts() {
            return this.$site.pages
            .filter((page) => page.path.startsWith("/blog/"));
        }
    },
}
</script>
