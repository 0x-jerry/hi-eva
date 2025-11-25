import TDesign from 'tdesign-vue-next'
import { createApp } from 'vue'
import App from './App.vue'
import { i18n } from './logic/i18n'
import { router } from './router'

import 'normalize.css'
import './styles/index.css'
import 'tdesign-vue-next/es/style/index.css';
import 'uno.css'

createApp(App)
  //
  .use(i18n)
  .use(router)
  .use(TDesign)
  .mount('#app')
