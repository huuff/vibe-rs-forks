# 📝 Documentation Maintenance Guide

This guide explains how to maintain and update the fake-rs documentation site.

## 🎯 Quick Updates

The documentation now uses **data-driven architecture**, making maintenance much easier!

### Adding a New Faker

Edit [`docs/data/fakers.js`](data/fakers.js):

```javascript
{
    id: "faker-newcategory",    // Unique ID for the category
    icon: "🎨",                   // Emoji icon
    title: "New Category",        // Display title
    fakers: [
        {
            code: "NewFaker()",   // Faker function signature
            desc: "Description"   // What it does
        },
        // Add more fakers...
    ],
    note: "⚠️ Requires <code>feature_name</code> feature"  // Optional note
}
```

**That's it!** The site will automatically:
- Display the new category
- Include it in search
- Apply proper styling
- Add animations

### Adding a New Example

Edit [`docs/data/examples.js`](data/examples.js):

```javascript
{
    title: "Example Title",
    language: "rust",  // or "bash", "toml", etc.
    code: `your code here`,
    note: "⚠️ Optional note"  // Optional
}
```

The site will automatically:
- Render the code block
- Apply syntax highlighting
- Add copy button
- Display any notes

### Adding a New Locale

Edit [`docs/data/locales.js`](data/locales.js):

```javascript
{
    name: "Finnish",       // Language name
    code: "FI_FI"          // Locale code
}
```

### Adding a New Feature Flag

Edit [`docs/data/features.js`](data/features.js):

```javascript
{
    code: "new_feature",              // Feature flag name
    description: "What it enables"    // Clear description
}
```

## 📂 File Structure

```
docs/
├── index.html           # Main HTML (mostly static now)
├── styles.css           # Styling (no changes needed for data)
├── script.js            # Dynamic content loader
├── data/
│   ├── locales.js      # ⭐ Edit this to add/update locales
│   ├── features.js     # ⭐ Edit this to add/update features
│   ├── fakers.js       # ⭐ Edit this to add/update fakers
│   └── examples.js     # ⭐ Edit this to add/update examples
└── README.md           # Documentation guide
```

## 🔧 Maintenance Tasks

### 1. Adding a Faker to an Existing Category

Find the category in `data/fakers.js` and add to the `fakers` array:

```javascript
{
    id: "faker-name",
    // ...
    fakers: [
        // Existing fakers...
        { code: "NewFaker()", desc: "New faker description" }  // ← Add here
    ]
}
```

### 2. Creating a New Faker Category

Add a new object to the `fakersData` array in `data/fakers.js`:

```javascript
const fakersData = [
    // Existing categories...
    {
        id: "faker-mycategory",
        icon: "🎯",
        title: "My Category",
        fakers: [
            { code: "MyFaker()", desc: "Does something cool" }
        ]
    }
];
```

### 3. Adding a New Locale

Add a new object to the `localesData` array in `data/locales.js`:

```javascript
const localesData = [
    // Existing locales...
    { flag: "🇫🇮", name: "Finnish", code: "FI_FI" }
];
```

### 4. Adding a New Feature Flag

Add a new object to the `featuresData` array in `data/features.js`:

```javascript
const featuresData = [
    // Existing features...
    { code: "new_feature", description: "Description of new feature" }
];
```

### 5. Updating Examples

**Simple:** Edit `data/examples.js` and modify the code or add new examples.

**Complex:** If you need to change example formatting, edit the `loadExamples()` function in `script.js`.

## 🎨 Styling Updates

### Changing Colors

Edit `styles.css` and update CSS variables:

```css
:root {
    --primary-color: #ff6b35;    /* Main accent color */
    --secondary-color: #004e89;  /* Secondary color */
    --accent-color: #1a659e;     /* Links and highlights */
    /* ... more variables */
}
```

### Adjusting Layout

Grid layouts are defined in `styles.css`:

```css
.faker-grid {
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
}
```

## 🧪 Testing Changes

### Local Testing

1. Open `index.html` in a browser
2. Or use a local server:
   ```bash
   # Python
   python -m http.server 8000

   # Node.js
   npx http-server

   # PHP
   php -S localhost:8000
   ```
3. Visit `http://localhost:8000`

### Check These

- ✅ All locales display correctly
- ✅ All feature flags display correctly
- ✅ All fakers display correctly
- ✅ Search functionality works
- ✅ Code examples are highlighted
- ✅ Copy buttons work
- ✅ Mobile menu functions
- ✅ No console errors (F12)

## 📋 Common Tasks Checklist

### When a New Faker is Added to fake-rs

1. [ ] Add to appropriate category in `data/fakers.js`
2. [ ] Include function signature with proper types
3. [ ] Write clear, concise description
4. [ ] Add note if requires specific feature flag
5. [ ] Test that search finds it
6. [ ] Add usage example to `data/examples.js` (if warranted)
7. [ ] Commit and push changes
8. [ ] Verify on live site

### When a New Feature Flag is Added

1. [ ] Add to `data/features.js`
2. [ ] Update any affected fakers with notes
3. [ ] Add example if it's a major feature
4. [ ] Update `README.md` if needed
5. [ ] Commit and push

### When a New Locale is Added

1. [ ] Add to `data/locales.js` with flag emoji and code
2. [ ] Test that examples work with new locale
3. [ ] Commit and push

## 🔍 Debugging

### Locales Not Showing

1. Check browser console for errors
2. Verify `data/locales.js` loads:
   ```javascript
   console.log(localesData);  // Should show array
   ```
3. Check for syntax errors in `locales.js`
4. Ensure `#locales-container` exists in HTML

### Features Not Showing

1. Check browser console for errors
2. Verify `data/features.js` loads:
   ```javascript
   console.log(featuresData);  // Should show array
   ```
3. Check for syntax errors in `features.js`
4. Ensure `#features-container` exists in HTML

### Fakers Not Showing

1. Check browser console for errors
2. Verify `data/fakers.js` loads:
   ```javascript
   console.log(fakersData);  // Should show array
   ```
3. Check for syntax errors in `fakers.js`
4. Ensure `#fakers-container` exists in HTML

### Examples Not Showing

1. Check console for errors
2. Verify `data/examples.js` loads:
   ```javascript
   console.log(examplesData);  // Should show array
   ```
3. Check for syntax errors in `examples.js`
4. Ensure `#examples-container` exists in HTML

### Search Not Working

1. Check that fakers rendered properly
2. Open console and look for errors
3. Verify `createSearchBox()` function runs
4. Test with simple query like "name"

## 📦 Data File Format Reference

### locales.js Structure

```javascript
const localesData = [
    {
        name: string,    // Language name (e.g., "English")
        code: string     // Locale code (e.g., "EN")
    }
];
```

### features.js Structure

```javascript
const featuresData = [
    {
        code: string,           // Feature flag name
        description: string     // What the feature enables
    }
];
```

### fakers.js Structure

```javascript
const fakersData = [
    {
        id: string,           // Unique identifier (e.g., "faker-name")
        icon: string,         // Emoji for category
        title: string,        // Display title
        fakers: [
            {
                code: string,   // Function signature
                desc: string    // Description
            }
        ],
        note: string | undefined  // Optional requirement note
    }
];
```

### examples.js Structure

```javascript
const examplesData = [
    {
        title: string,        // Example title
        language: string,     // "rust", "bash", "toml"
        code: string,         // Code content
        note: string | undefined  // Optional note
    }
];
```

## 🚀 Deployment

After making changes:

```bash
git add docs/
git commit -m "Update documentation: [describe changes]"
git push origin main
```

GitHub Pages will automatically update in 1-2 minutes.

## 💡 Tips

### Organizing Fakers

- Keep categories in logical order (most common first)
- Group related fakers together within categories
- Use clear, consistent naming
- Include type information in signatures

### Writing Descriptions

- Start with action verb (Generate, Create, etc.)
- Be concise but clear
- Explain what, not how
- Include key details (format, range, etc.)

### Code Examples

- Show real-world usage
- Include imports
- Add comments for clarity
- Keep examples focused
- Show output when helpful

### Feature Notes

- Be consistent with format
- Use warning emoji: ⚠️
- Wrap feature names in `<code>` tags
- Place notes after faker grid

## 🆘 Need Help?

1. Check browser console for errors
2. Review this guide
3. Look at existing examples in data files
4. Test in private/incognito window
5. Clear browser cache
6. Check GitHub Pages build status

## 📈 Future Improvements

Ideas for enhancing maintainability:

- [ ] Add JSON schema validation for data files
- [ ] Create CLI tool to add fakers
- [ ] Auto-generate from Rust source
- [ ] Add TypeScript for type safety
- [ ] Create contribution templates
- [ ] Add automated testing

## ✅ Benefits of Data-Driven Architecture

✨ **Easy Updates**: Edit one file instead of complex HTML
🔍 **Searchable**: All data automatically included in search
🎨 **Consistent**: Styling applied automatically
⚡ **Fast**: Add multiple fakers quickly
🐛 **Fewer Errors**: Less copy-paste, clearer structure
📝 **Maintainable**: Clear separation of data and presentation
🚀 **Scalable**: Easy to add 10 or 100 fakers

---

For questions or issues, open an issue on the [fake-rs repository](https://github.com/cksac/fake-rs).
