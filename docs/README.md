# GitHub Pages Deployment

To enable GitHub Pages for this project:

1. Go to your repository: https://github.com/chamorrowizard/bible-headlines
2. Click on **Settings** (top right)
3. Scroll down to **Pages** in the left sidebar
4. Under **Source**, select:
   - Branch: `main`
   - Folder: `/docs`
5. Click **Save**

GitHub will automatically deploy your site. It will be available at:
**https://chamorrowizard.github.io/bible-headlines/**

## What's Deployed

- `index.html` - Static web application with all features
- `data.json` - Complete Bible data (all 66 books)

The static site includes:
- ✅ All 66 Bible books
- ✅ Real-time search functionality
- ✅ Testament filtering
- ✅ Collapsible book sections
- ✅ Mobile-responsive design
- ✅ All original features (client-side only)

## Local Testing

To test the static site locally:

```bash
cd docs
python3 -m http.server 8000
```

Then visit: http://localhost:8000

## Updates

Whenever you update the Rust application:

1. Run the server: `cargo run`
2. Export the full HTML: `curl http://localhost:8080 > docs/index.html`
3. Commit and push changes: `git add docs/ && git commit -m "Update content" && git push`
4. GitHub Pages will auto-deploy in 1-2 minutes
