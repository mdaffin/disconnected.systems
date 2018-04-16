<template>
  <router-link :to="to">
    <article>
      <div class="title">{{ title }}</div>
      <div class="date">{{ date.toLocaleDateString() }}</div>
      <div class="description">{{ description }}</div>
    </article>
  </router-link>
</template>

<style scoped>
article {
    color: #2c3e50;
    padding: 16px;
    border-bottom: 1px solid #fff;
    border-top: 1px solid #fff;
}

article:hover {
    border-bottom: 1px solid #eaecef;
    border-top: 1px solid #eaecef;

}
.title {
    font-weight: bold;
    margin-bottom: 8px
}
.date {
    font-size: small;
    font-style: italic;
    margin-bottom: 4px
}
.description {
}
</style>

<script>
export default {
    props: ["title", "to", "date", "description"]
}
</script>
