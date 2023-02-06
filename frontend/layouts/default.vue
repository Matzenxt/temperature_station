<script lang="ts">
import Header from "~/components/Header.vue";

export default {
  name: 'DefaultLayout',
  components: {Header}
}
</script>

<template>
  <v-app>

    <Header></Header>

    <slot />

  </v-app>
</template>
