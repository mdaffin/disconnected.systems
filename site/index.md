---
home: true
---
# A title

<div v-for="post in posts">
  <h3>{{ post.title }}</h3>
</div>

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
