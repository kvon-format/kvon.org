/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
		"./html-generated/**/*.html",
		"./js/**/*.js",
	],
	theme: {
		extend: {
			typography: (theme) => ({
				DEFAULT: {
					css: {
						a: {
							textDecoration: 'none',
							color: theme('colors.blue.500'),
							fontWeight: '600',
							'$:hover': {
								textDecoration: 'underline'
							}
						}
					}
				}
			})
		},
	},
	plugins: [
		require('@tailwindcss/typography')
	],
}