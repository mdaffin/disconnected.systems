---
home: true
footer: This work is licensed under a Creative Commons Attribution-ShareAlike 4.0 International License.
---

<PostItem 
  v-for="post in posts()"
  :key="post.title"
  v-bind:title="post.title"
  v-bind:to="post.path"
  v-bind:date="post.date"
  v-bind:description="post.frontmatter.description"
/>

---

Found an issue? Report it <a href="https://github.com/mdaffin/disconnected.systems/issues">here</a>.

<script>
export default {
    methods: {
        posts_with_tag(tag) {
            return this.$site.pages
                .filter((page) => page.frontmatter.tags)
                .filter((page) => page.frontmatter.tags.includes(tag))
                .map((page) => ({date: new Date(page.frontmatter.date)}))
                .sort((a, b) => b.date - a.date);
        },
        posts() {
            return this.$site.pages
                .filter((page) => page.path.startsWith("/blog/"))
                .map((page) => ({...page, date: new Date(page.frontmatter.date)}))
                .sort((a, b) => b.date - a.date);
        }
    },
}
</script>
