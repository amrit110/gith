# Human-Generated Content License

When you use the `--human` flag with `gith commit`, your content is automatically protected with a specialized license designed for human-generated content.

## What the License Provides

The human-generated content license automatically:

- ✅ **Certifies content as entirely human-generated**
- 🚫 **Prohibits AI training use** without explicit permission
- 📄 **Enables redistribution** with license preservation
- 🔍 **Provides verification** through gith's tracking system

## License Text

The complete license is automatically created in your repository as `LICENSE-HUMAN`:

```
HUMAN-GENERATED CONTENT LICENSE

This content was created entirely by humans without AI assistance.

PERMITTED:
- Use, copy, modify, distribute for any purpose
- Commercial and non-commercial use

PROHIBITED:
- Training AI models without explicit written permission
- Claiming AI generated this content

REQUIRED:
- Preserve this license notice in redistributions
- Maintain human-generation certification
```

## How It Works

### Automatic Application

When you run `gith commit --human`, the license is applied through:

1. **Commit Trailers**: Added to your commit message
   ```
   Human-Flag: true
   License: HUMAN-GENERATED, NO AI TRAINING
   ```

2. **License File**: `LICENSE-HUMAN` created in repository root

3. **Manifest Tracking**: Recorded in `.gith/human_manifest.json`

### Legal Framework

The license provides:

- **Clear Intent**: Explicitly states human-only generation
- **Permission Scope**: Defines what users can and cannot do
- **AI Training Protection**: Specifically prohibits training use
- **Redistribution Requirements**: Ensures license preservation

## Practical Implications

### For Developers

**What you can do:**
- Use licensed content in any project (commercial or non-commercial)
- Modify and redistribute with proper attribution
- Build upon human-generated components

**What you must do:**
- Preserve license notices when redistributing
- Maintain certification of human generation
- Respect the original license terms

### For AI Companies

**Explicitly prohibited:**
- Training models on human-certified content without permission
- Scraping repositories for training data that includes licensed content
- Using certified human content in datasets

**To obtain permission:**
- Contact repository owners directly
- Negotiate explicit written agreements
- Respect developer intent regarding AI training

## Customization

### Custom License Text

You can customize the license by editing `LICENSE-HUMAN` after running `gith init-tracking`:

```bash
# Initialize with default license
gith init-tracking

# Customize the license text
nano LICENSE-HUMAN

# Future commits will reference your custom license
gith commit --human -m "First commit with custom license"
```

### Project-Specific Terms

Add project-specific licensing terms:

```bash
# Add additional terms to LICENSE-HUMAN
cat >> LICENSE-HUMAN << 'EOF'

ADDITIONAL TERMS:
- Attribution required in derivative works
- Commercial use requires notification to authors
- Research use permitted with citation
EOF
```

## Integration Examples

### In README Files

Document your licensing approach:

```markdown
## Human-Generated Content

This project uses `gith` to track and license human-generated content.

- View certified human contributions: `gith list-human`
- Licensed under HUMAN-GENERATED CONTENT LICENSE
- See LICENSE-HUMAN for full terms
```

### In Contributing Guidelines

Set expectations for contributors:

```markdown
## Contributing

When contributing original work:
- Use `gith commit --human` for human-generated content
- Regular commits for AI-assisted work are welcome
- All human-certified content falls under our human-generation license
```

## Legal Considerations

### Enforceability

The license provides:
- **Clear Notice**: Explicit human-generation claims
- **Specific Prohibitions**: Targeted at AI training use
- **Standard Terms**: Based on established licensing patterns

### Compliance

To respect licensed content:
- **Check for gith tracking** in repositories you use
- **Honor license terms** when redistributing
- **Seek permission** for AI training use
- **Maintain attributions** in derivative works

### Liability

The license includes standard disclaimers:
- Content provided "as is"
- No warranty of fitness for purpose
- Limited liability for use

## Frequently Asked Questions

### Can I change the license after committing?

The license is embedded in commit history and should not be changed retroactively. You can:
- Customize future commits by editing `LICENSE-HUMAN`
- Add clarifying documentation
- Contact legal counsel for significant changes

### Does this affect standard software licenses?

No. The human-generation license works alongside:
- MIT, Apache, GPL licenses for code
- Creative Commons for documentation
- Project-specific licenses

### What about collaborative contributions?

For team contributions:
- Each contributor certifies their own content
- Mixed commits can include multiple certifications
- Team policies should define human-generation standards

### How does this relate to copyright?

- **Copyright remains** with original authors
- **License grants permissions** for use and redistribution
- **Human certification** adds additional protections
- **Standard copyright law** still applies

## Verification and Compliance

### Verifying Human-Generated Content

```bash
# Check if repository uses gith
ls -la .gith/

# View human-certified commits
gith list-human --commits-only

# Examine commit details
git show --show-signature <commit-hash>
```

### Compliance Scanning

For automated compliance checking:

```bash
#!/bin/bash
# Check for human-generated content licensing
if [ -f "LICENSE-HUMAN" ]; then
    echo "Repository contains human-generated content license"
    gith list-human --commits-only | wc -l | xargs echo "Human commits:"
else
    echo "No human-generation licensing detected"
fi
```

The human-generated content license provides a clear, enforceable framework for protecting authentic human contributions in the age of AI-assisted development.
