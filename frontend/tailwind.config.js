/** @type {import('tailwindcss').Config} */
module.exports = {
    mode: 'jit',
    content: ['./css/*.css','./src/**/*.rs', './index.html'
    ],
    plugins: [
        require('daisyui'),
        require('@tailwindcss/typography')
    ],
    theme: {
        fontFamily: {
            sans: ['Inter', 'sans-serif'],
            display: ['Comfortaa', 'display'],
            mono: ['Fira Mono', 'mono']
        },
    },
    daisyui: {
        themes: [
            "dracula",
            "synthwave",
            "dark",
            "light",
            "cupcake",
            "bumblebee",
            "emerald",
            "corporate",
            "retro",
            "cyberpunk",
            "valentine",
            "halloween",
            "garden",
            "forest",
            "aqua",
            "lofi",
            "pastel",
            "fantasy",
            "wireframe",
            "black",
            "luxury",
            "cmyk",
            "autumn",
            "business",
            "acid",
            "lemonade",
            "night",
            "coffee",
            "winter",
            "dim",
            "nord",
            "sunset",
         ],
    }
}
