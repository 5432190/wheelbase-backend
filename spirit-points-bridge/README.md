# Spirit Points Bridge

A static MVP landing page for a travel-service concept that helps travelers use
their own Spirit points as part of lower-cost complete trips.

The site is intentionally built without a backend so the idea can be tested
quickly with a landing page, a route-request form, and manual concierge
fulfillment.

## Files

- `index.html` - page structure and product messaging
- `styles.css` - responsive visual design
- `script.js` - client-side bridge strategy estimator

## Run locally

Open `index.html` in a browser, or serve the folder with any static server:

```bash
python3 -m http.server 8000
```

## Create the standalone GitHub repo

This project is designed to become its own repository.

1. Create a new empty GitHub repo named `spirit-points-bridge`.
2. Copy the contents of this folder into that new repo.
3. From inside the new repo, run:

```bash
git init
git add .
git commit -m "Launch Spirit Points Bridge MVP"
git branch -M main
git remote add origin git@github.com:<your-user-or-org>/spirit-points-bridge.git
git push -u origin main
```

## Deploy options

Because the MVP is plain HTML, CSS, and JavaScript, it can deploy without a
build step on:

- GitHub Pages
- Netlify
- Vercel
- Cloudflare Pages
