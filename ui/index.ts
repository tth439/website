import './scss/index.scss';

const getPreferredColorScheme = () => {
  const darkQuery = "(prefers-color-scheme: dark)";
  const darkMQL: MediaQueryList | null = window.matchMedia ? window.matchMedia(darkQuery) : null;
  if (darkMQL?.media === darkQuery && darkMQL?.matches)
    return "dark";
  return "default";
};

const colorScheme = localStorage.getItem("stored-color-scheme") || getPreferredColorScheme();
document.documentElement.setAttribute("data-color-scheme", colorScheme);

document.getElementById("darkmode-btn")!.onclick = () => {
  const colorScheme = document.documentElement.getAttribute("data-color-scheme");
  const newColorScheme = colorScheme === "default" ? "dark" : "default";
  document.documentElement.setAttribute("data-color-scheme", newColorScheme);
  localStorage.setItem("stored-color-scheme", newColorScheme);
};
