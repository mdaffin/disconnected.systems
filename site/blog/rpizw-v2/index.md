---
date: '2018-05-05T17:44:02+00:00'
description: An update to my rpizw rover
indexPage: true
---

# RPIZW Rover V2

The second version of my raspberry pi zero w powered rover.

<PostItem 
  v-for="post in posts()"
  :key="post.title"
  v-bind:title="post.title"
  v-bind:to="post.path"
  v-bind:date="post.date"
  v-bind:description="post.frontmatter.description"
/>

<script>
export default {
    methods: {
        posts() {
            console.log($page.path)
            return this.$site.pages
                .filter((page) => page.path.startsWith($page.path))
                .filter((page) => !page.frontmatter['indexPage'])
                .map((page) => ({...page, date: new Date(page.frontmatter.date)}))
                .sort((a, b) => b.date - a.date);
        }
    },
}
</script>

