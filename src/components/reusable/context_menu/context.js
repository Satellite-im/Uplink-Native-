document.getElementById("ID").addEventListener(
  "contextmenu",
  function (ev) {
    ev.stopPropagation()
    ev.preventDefault()
    const context_menu = document.getElementById("context-menu")
    // TODO: if this context menu would render off the bottom of the page, render the bottom left corner at mouse pointer.
    // If the menu would render off the right, render the opposide side, continue this logic for top left bottom and right
    // so that the menu never renders off screen.
    context_menu.classList.remove("hidden")
    context_menu.style.top = `${ev.pageY}px`
    context_menu.style.left = `${ev.pageX}px`
    return false
  },
  false,
)

document.addEventListener("click", (ev) => {
  const context_menu = document.getElementById("context-menu")
  context_menu.classList.add("hidden")
})