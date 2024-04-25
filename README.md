# Quick Aliases

`The aliases are stored in the json file by path:
$HOME/.config/quick-aliases/aliases.json`

## 📦 Installation:
Arch linux. From AUR https://aur.archlinux.org/packages/quick-aliases
``` bash
yay -S quick-aliases
# Or
paru -S quick-aliases
```

## 🚀 Usage:
``` bash
# Add new alias
quick-aliases add [NAME] [COMMAND]

# Remove alias
quick-aliases rm [NAME]

# Aliases list
quick-aliases ls

# Remove all aliases
quick-aliases rma

# Execute the alias command
quick-aliases [NAME]

quick-aliases help
```

Example:
``` bash
quick-aliases add postgresdb pgcli -h localhost -p 5432 -U postgres -W  
quick-aliases postgresdb
quick-aliases rm postgresdb
```
