import Aura from '@primeuix/themes/aura'
import PrimeVue from 'primevue/config'
import { createApp } from 'vue'
import App from './App.vue'
import { router } from './router'

import 'normalize.css'
import 'uno.css'

createApp(App)
  .use(router)
  .use(PrimeVue, {
    theme: {
      preset: Aura,
    },
  })
  .mount('#app')
