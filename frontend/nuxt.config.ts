// https://nuxt.com/docs/api/configuration/nuxt-config
import vuetify from "~/plugins/vuetify";

export default defineNuxtConfig({
  app: {
    head: {
      title: "Temperatur Station",
    },
  },

  // typescripts
  typescript: {
    strict: true,
    typeCheck: true,
  },

  modules: [
    '@vueuse/nuxt',
  ],

  css: ['vuetify/styles/main.sass',
      '@mdi/font/css/materialdesignicons.css'
  ],

  build: {
    transpile: ['vuetify'],
  },
})
