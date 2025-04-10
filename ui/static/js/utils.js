let bytes_sizes = ["bytes", "KB", "MB", "GB"];
let number_sizes = ["", "K", "M", "B", "T", "Qa"];
let allowed_extensions = [".gif", ".jpg", ".jpeg", ".png", ".webp"];

let default_name_length = 35;

const clamp = (val, min, max) => Math.min(Math.max(val, min), max);

function format_size(size) {
  size = Number(size);
  if (isNaN(size)) return "Invalid number";

  let steps = 0;
  while (size >= 1024 && steps < bytes_sizes.length - 1) {
    size /= 1024;
    steps += 1;
  }
  return size.toFixed(2) + " " + bytes_sizes[steps];
}
function format_number(number) {
  number = Number(number);
  if (isNaN(number)) return "Invalid number";

  let steps = 0;
  while (number >= 10000 && steps < number_sizes.length - 1) {
    number /= 1000;
    steps += 1;
  }
  return (number.toFixed(2) + "" + number_sizes[steps]).replace(".00", "");
}
function formatWithZero(number) {
  number = Number(number);
  if (isNaN(number)) return "Invalid number";

  return String(number).padStart(2, "0");
}
function shorten_string(input, max_size) {
  if (max_size == null) {
    max_size = default_name_length;
  }
  if (input.length <= max_size) {
    return input;
  }
  return (
    input.slice(0, max_size / 2) +
    " ... " +
    input.slice(input.length - max_size / 2)
  ); // Use slice to get the last `max_size` characters
}

function format_date(date) {
  return (
    formatWithZero(date.getHours()) +
    ":" +
    formatWithZero(date.getMinutes()) +
    ":" +
    formatWithZero(date.getSeconds()) +
    " " +
    formatWithZero(date.getDate()) +
    "." +
    formatWithZero(date.getMonth()) +
    "." +
    date.getFullYear()
  );
}
function is_image(name) {
  let lower_name = name.toLowerCase();

  for (let index = 0; index < allowed_extensions.length; index++) {
    const extension = allowed_extensions[index];

    if (lower_name.endsWith(extension) == true) {
      return true;
    }
  }

  return false;
}
