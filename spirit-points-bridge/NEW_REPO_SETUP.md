# New repository setup

Use these steps to move Spirit Points Bridge into its own GitHub repository.

## 1. Create the new repo

Create an empty GitHub repository named:

```text
spirit-points-bridge
```

Do not initialize it with a README, license, or `.gitignore` if you plan to push
this folder directly.

## 2. Copy the project files

Copy everything inside this folder into the new repository root:

```text
spirit-points-bridge/
  .gitignore
  index.html
  README.md
  script.js
  styles.css
```

## 3. Push the first commit

Run these commands from inside the new repository folder:

```bash
git init
git add .
git commit -m "Launch Spirit Points Bridge MVP"
git branch -M main
git remote add origin git@github.com:<your-user-or-org>/spirit-points-bridge.git
git push -u origin main
```

## 4. Deploy

Because this MVP has no build step, point GitHub Pages, Netlify, Vercel, or
Cloudflare Pages at the repository root.
