import { createApp } from 'vue';
import App from './App.vue';
import { monaco } from './monaco';

const app = createApp(App);
app.use(monaco);
app.mount('#app');
