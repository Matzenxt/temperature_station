// https://nuxt.com/docs/api/configuration/nuxt-config
import vuetify from "~/plugins/vuetify";

export default defineNuxtConfig({

  ssr: true,

  // typescripts
  typescript: {
    strict: true,
    typeCheck: true,
  },

  css: ['vuetify/styles/main.sass',
      '@mdi/font/css/materialdesignicons.css'
  ],

  build: {
    transpile: ['vuetify'],
  },
})
