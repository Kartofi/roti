let bytes_sizes = ["bytes", "KB", "MB", "GB"];

function size_format(size) {
  let steps = 0;
  while (size >= 1024 && steps < bytes_sizes.length - 1) {
    size /= 1024;
    steps += 1;
  }
  return size.toFixed(2) + " " + bytes_sizes[steps];
}
