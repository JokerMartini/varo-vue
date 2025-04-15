export default defineNuxtConfig({
 devtools: { enabled: false },
 ssr: false,
 srcDir: "src",
 css: ['~/assets/css/main.css'],
 routeRules: {
	"/": { prerender: true }
},
 modules: [
	"@nuxt/ui",
	'@pinia/nuxt'
],
 compatibilityDate: "2025-04-10"
});