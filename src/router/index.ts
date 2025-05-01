import { createRouter, createWebHistory } from 'vue-router';
import TextFileViewer from './components/TextFileViewer.vue';
import Home from './components/Home.vue'; // Assume you have a Home component

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/log-files',
    name: 'LogFiles',
    component: TextFileViewer
  }
  // Add other routes as needed
];

const router = createRouter({
  history: createWebHistory(),
  routes
});

export default router;