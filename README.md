![gh-pages-publish](https://github.com/andi-mache/miniature-train//actions/workflows/publish_gh_pages.yml/badge.svg)

## Test project for yew hosted on github.io

[About github.io](https://pages.github.com/)  

```
main branch
├── index.html
│
├── css                       # css/scss
│   ├── index.css
│   ├── index.scss
│   ├── main.scss
│   └── tailwind
│       └── tailwind_base.css
│
├── src                       # yew here
│   └── main.rs
│
├── static
│   └── 404.html
│
├── README.md
├── Cargo.lock
├── Cargo.toml
├── Trunk.toml
├── package.json
├── tailwind.config.js
└── webpack.config.js

gh-pages branch
├── index.html
├── 404.html
├── index-7823f64dca35d01f.js
├── index-7823f64dca35d01f_bg.wasm
└── main-baa4389a63fdea1b.css
```


## Using Custom Domain

go to `./.github/workflows/publish_gh_pages.yml` and add your domain in cname field.  

## Routing

Using `static/404.html`. for more information, checkout https://github.com/rafgraph/spa-github-pages.  
