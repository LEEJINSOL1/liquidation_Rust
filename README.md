# LiquidScope-rs
온체인+오프체인 선물거래 분석 플랫폼(MVP).

```
LiquidScope-rs
├─ crates/
│  ├─ storage      # DB 커넥터/리포지터리
│  ├─ ingest       # 거래소 피드
│  ├─ onchain      # EVM 인터페이스
│  ├─ analytics    # 지표 계산
│  ├─ api          # API 라우트
│  └─ alert        # 웹훅 알림
├─ apps/
│  ├─ api-server   # API 서버 실행
│  └─ worker       # 주기 태스크
└─ migrations/     # SQL 스키마
```

## 빠른 시작

```bash
docker-compose up -d
cp .env.example .env
sqlx migrate run
cargo run -p api-server
```

### 주요 엔드포인트
- `GET /health`
- `GET /spread?base=hyperliquid&ref=binance&symbol=BTCUSDT`
- `GET /liq-risk?symbol=BTC-PERP`

## 제한사항 / TODO
- 실거래소 프로토콜 파서는 스텁 상태
- 온체인 인덱싱은 기본 인터페이스만
- 분석 로직은 최소한의 예시 구현

![placeholder](assets/screenshot.png)
