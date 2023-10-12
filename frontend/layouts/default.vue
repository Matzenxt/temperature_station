<script lang="ts">

export default {
  name: 'DefaultLayout',
}
</script>

<template>
  <div>
    <v-card>
      <v-layout>
        <v-navigation-drawer
            permanent
            width="150"
        >
          <v-list
              density="compact"
              nav
          >
            <v-list-item>
              <NuxtLink to="/">Dashboard</NuxtLink>
            </v-list-item>
            <v-list-item>
              <NuxtLink to="/rooms">Räume</NuxtLink>
            </v-list-item>
          </v-list>
        </v-navigation-drawer>
        <v-main >
          <slot />
        </v-main>
      </v-layout>
    </v-card>
  </div>
</template>
