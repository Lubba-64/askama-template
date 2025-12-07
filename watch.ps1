Start-Process -NoNewWindow -FilePath cargo -ArgumentList "watch -x run" ; npx tailwindcss@3.4.17 -i public/style.css -o public/tw.css --watch -c tailwind.config.js
