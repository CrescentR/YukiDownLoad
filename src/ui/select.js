/**
 * Themed dropdowns.
 *
 * The native `<select>` popup is drawn by the OS and ignores the app's theme,
 * so every select is paired with a button + listbox drawn here. The native
 * element stays in the DOM (hidden) as the source of truth: existing handlers
 * keep reading `data-setting` / `data-quality-preset` and listening for
 * `input`/`change`, which this module dispatches on selection.
 *
 * `enhanceSelects()` is idempotent and is called after every render, which
 * both wraps newly rendered selects and re-syncs labels whose value or option
 * text was changed programmatically.
 */

import { escapeHtml } from "../core/util.js";
import { icon } from "./icons.js";

const triggers = new WeakMap();
let open = null; // { select, trigger, menu, active }

export function enhanceSelects(root = document) {
  for (const select of root.querySelectorAll("select:not([data-enhanced])")) wrap(select);
  for (const select of root.querySelectorAll("select[data-enhanced]")) sync(select);
  if (open && !open.trigger.isConnected) close(false);
}

function wrap(select) {
  select.dataset.enhanced = "";
  select.hidden = true;
  select.tabIndex = -1;
  const trigger = document.createElement("button");
  trigger.type = "button";
  trigger.className = `${select.className} select-trigger`;
  // Lets re-renders find the trigger again to restore keyboard focus.
  if (select.dataset.setting) trigger.dataset.selectFor = select.dataset.setting;
  trigger.setAttribute("aria-haspopup", "listbox");
  trigger.setAttribute("aria-expanded", "false");
  trigger.innerHTML = `<span class="select-value"></span>${icon("chevron-down", { size: 14, class: "select-chevron" })}`;
  trigger.addEventListener("click", () => (open?.select === select ? close() : show(select)));
  trigger.addEventListener("keydown", (event) => onKey(event, select));
  select.after(trigger);
  triggers.set(select, trigger);
  sync(select);
}

function sync(select) {
  const trigger = triggers.get(select);
  if (!trigger) return;
  const label = select.selectedOptions[0]?.textContent ?? "";
  const value = trigger.firstElementChild;
  if (value.textContent !== label) value.textContent = label;
  trigger.title = label;
  trigger.disabled = select.disabled;
  const aria = select.getAttribute("aria-label");
  if (aria) trigger.setAttribute("aria-label", aria);
}

function show(select) {
  close(false);
  const trigger = triggers.get(select);
  const menu = document.createElement("div");
  menu.className = "select-menu";
  menu.setAttribute("role", "listbox");
  menu.innerHTML = [...select.options]
    .map(
      (option, index) => `
      <div class="select-option${option.selected ? " selected" : ""}" role="option" data-index="${index}"
           aria-selected="${option.selected}"${option.disabled ? ' aria-disabled="true"' : ""}>
        <span>${escapeHtml(option.textContent)}</span>${icon("check", { size: 13, class: "select-check" })}
      </div>`,
    )
    .join("");
  // Keep focus on the trigger so keyboard handling stays in one place.
  menu.addEventListener("mousedown", (event) => event.preventDefault());
  menu.addEventListener("click", (event) => {
    const item = event.target.closest(".select-option");
    if (item && item.getAttribute("aria-disabled") !== "true") choose(Number(item.dataset.index));
  });
  menu.addEventListener("mousemove", (event) => {
    const item = event.target.closest(".select-option");
    if (item) highlight(Number(item.dataset.index), false);
  });
  document.body.append(menu);
  open = { select, trigger, menu, active: -1 };
  trigger.setAttribute("aria-expanded", "true");
  trigger.classList.add("open");
  place();
  highlight(Math.max(0, select.selectedIndex), true);
}

function place() {
  const { trigger, menu } = open;
  const rect = trigger.getBoundingClientRect();
  const gap = 4;
  const margin = 8;
  menu.style.minWidth = `${rect.width}px`;
  const below = window.innerHeight - rect.bottom - gap - margin;
  const above = rect.top - gap - margin;
  const height = menu.scrollHeight;
  const up = height > below && above > below;
  menu.style.maxHeight = `${Math.min(280, up ? above : below)}px`;
  menu.style.top = up ? "" : `${rect.bottom + gap}px`;
  menu.style.bottom = up ? `${window.innerHeight - rect.top + gap}px` : "";
  menu.classList.toggle("up", up);
  const left = Math.min(rect.left, window.innerWidth - menu.offsetWidth - margin);
  menu.style.left = `${Math.max(margin, left)}px`;
}

function close(refocus = true) {
  if (!open) return;
  const { trigger, menu } = open;
  open = null;
  menu.remove();
  trigger.setAttribute("aria-expanded", "false");
  trigger.classList.remove("open");
  if (refocus && trigger.isConnected) trigger.focus({ preventScroll: true });
}

function highlight(index, scroll) {
  const items = open.menu.children;
  if (!items[index]) return;
  items[open.active]?.classList.remove("active");
  items[index].classList.add("active");
  open.active = index;
  if (scroll) items[index].scrollIntoView({ block: "nearest" });
}

function step(delta) {
  const options = open.select.options;
  let index = open.active;
  for (let i = 0; i < options.length; i += 1) {
    index = (index + delta + options.length) % options.length;
    if (!options[index].disabled) break;
  }
  highlight(index, true);
}

function choose(index) {
  const { select } = open;
  close();
  if (select.selectedIndex === index) return;
  select.selectedIndex = index;
  sync(select);
  select.dispatchEvent(new Event("input", { bubbles: true }));
  select.dispatchEvent(new Event("change", { bubbles: true }));
}

function onKey(event, select) {
  const isOpen = open?.select === select;
  const key = event.key;
  if (!isOpen) {
    if (["ArrowDown", "ArrowUp", "Enter", " "].includes(key)) {
      event.preventDefault();
      show(select);
    }
    return;
  }
  // Swallow keys so Escape closes only the menu, not the surrounding modal.
  if (key === "Escape" || key === "Tab") {
    if (key === "Escape") {
      event.preventDefault();
      event.stopPropagation();
    }
    close(key === "Escape");
  } else if (key === "ArrowDown" || key === "ArrowUp") {
    event.preventDefault();
    step(key === "ArrowDown" ? 1 : -1);
  } else if (key === "Home" || key === "End") {
    event.preventDefault();
    highlight(key === "Home" ? 0 : select.options.length - 1, true);
  } else if (key === "Enter" || key === " ") {
    event.preventDefault();
    event.stopPropagation();
    choose(open.active);
  }
}

document.addEventListener(
  "pointerdown",
  (event) => {
    if (open && !open.menu.contains(event.target) && !open.trigger.contains(event.target)) close(false);
  },
  true,
);
document.addEventListener(
  "scroll",
  (event) => {
    if (open && !open.menu.contains(event.target)) close(false);
  },
  true,
);
window.addEventListener("resize", () => close(false));
window.addEventListener("blur", () => close(false));
