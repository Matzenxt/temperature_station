// https://nuxt.com/docs/api/configuration/nuxt-config
import vuetify from "~/plugins/vuetify";

export default defineNuxtConfig({
  ssr: false,

  app: {
    head: {
      title: "Temperatur Station",
    },
  },

  runtimeConfig: {
    public: {
      url: process.env.SERVER_ADDRESS || 'http://localhost',
      port: process.env.SERVER_PORT || '9090'
    },
  },

  // typescripts
  typescript: {
    strict: true,
    typeCheck: true,
  },

  modules: [
    '@vueuse/nuxt',
    '@pinia/nuxt',
  ],

  css: ['vuetify/styles/main.sass',
      '@mdi/font/css/materialdesignicons.css'
  ],

  build: {
    transpile: ['vuetify'],
  },

  compatibilityDate: '2024-09-08',
})
