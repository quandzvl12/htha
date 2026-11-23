# Công cụ đo hiệu năng (bước 3)
    cd rust/tools/perf && npm i jsdom@24 @napi-rs/canvas
    # số lệnh canvas + thời gian JS mỗi khung; Q = mức đồ họa 0/1/2, CLS = nghề, SKILLS=1 = bắn kỹ năng liên tục
    ATTR=0 Q=0 CLS=summoner SKILLS=1 node bench.js ../../../index.html 500 3000
Lưu ý: đây là canvas giả đếm lệnh + đồng hồ ảo (không phải GPU điện thoại). Số liệu dùng để SO SÁNH trước/sau, không phải FPS thật.
`bench.js` chèn một hook gỡ lỗi vào BẢN SAO trong bộ nhớ của index.html, không sửa file thật.
