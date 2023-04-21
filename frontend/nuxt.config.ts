// https://nuxt.com/docs/api/configuration/nuxt-config
import vuetify from "~/plugins/vuetify";

export default defineNuxtConfig({
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

  // TODO: Remove when pina got it working!
  alias: {
    pinia: "/node_modules/@pinia/nuxt/node_modules/pinia/dist/pinia.mjs"
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
})
