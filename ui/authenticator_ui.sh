OPTIONS=$(authenticator list)
NEW_ITEM="+ new item"
COPY="copy"
SHOW="show"

new_item() {
  NAME=$(echo "" | dmenu -p "Name:")
  foot sh -c "authenticator add $NAME"
}

item_menu(){
  ACTION=$(echo -en "$COPY\n$SHOW" | dmenu -l 10)
  if [ "$ACTION" = "$COPY" ]; then
    authenticator show $SELECTED | xclip -sel c
  elif [ "$ACTION" = "$SHOW" ]; then
    foot sh -c "watch -n 1 authenticator show $SELECTED"
  fi
}

SELECTED=$(echo -en "$OPTIONS\n$NEW_ITEM" | dmenu -l 10)
case "$SELECTED" in
    "$NEW_ITEM") new_item ;;
    *) item_menu ;;
esac
