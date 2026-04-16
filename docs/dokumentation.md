# Marvels – Authentifizierung & Autorisierung

## OAuth 2.1 Flow mit PKCE (RFC 7636)

### Übersicht

Das Projekt implementiert den **Authorization Code Flow** gemäß OAuth 2.1
mit PKCE-Pflicht für alle Clients.

---

### Ablauf

```
Client                              Server
  │                                    │
  │── POST /authenticate ──────────────▶│
  │   { client_id, client_secret,      │  1. client_id / client_secret prüfen (TODO: DB)
  │     code_challenge,                │  2. auth_code (UUID) generieren
  │     code_challenge_method="S256" } │  3. auth_code → code_challenge im Cache speichern
  │                                    │
  │◀── { subject: "<auth_code>" } ─────│
  │                                    │
  │── POST /authorize ─────────────────▶│
  │   { grant_type="authorization_code",│  4. auth_code aus Cache holen (einmalig!)
  │     code:        "<auth_code>",    │  5. PKCE prüfen:
  │     code_verifier,                 │     BASE64URL(SHA256(code_verifier)) == code_challenge
  │     scope }                        │  6. JWT Access Token ausstellen (HS256, 1h)
  │                                    │
  │◀── { access_token,                 │
  │      token_type: "Bearer",         │
  │      expires_in: 3600,             │
  │      scope }                       │
  │                                    │
  │── GET /protected ──────────────────▶│
  │   Authorization: Bearer <jwt>      │  7. JWT-Signatur und Ablaufzeit prüfen
  │                                    │  8. Claims auslesen (sub, scope)
  │◀── 200 OK                          │
      "Willkommen, <client_id>!        │
       Berechtigungen: <scope>"        │
```

---

### Endpunkte

| Methode | Pfad             | Beschreibung                                      |
|---------|------------------|---------------------------------------------------|
| POST    | `/authenticate`  | Identität prüfen, Authorization Code ausstellen   |
| POST    | `/authorize`     | Code einlösen, JWT Access Token ausstellen        |
| GET     | `/protected`     | Geschützte Ressource (Bearer Token erforderlich)  |

---

### PKCE (RFC 7636)

PKCE schützt den Authorization Code vor Abfangen (z.B. in mobilen Apps).

| Schritt | Wert                                         | Wer              |
|---------|----------------------------------------------|------------------|
| Generieren | `code_verifier` = 64 zufällige Bytes (Base64url) | Client        |
| Ableiten   | `code_challenge` = BASE64URL(SHA256(verifier))   | Client        |
| Senden     | `code_challenge` beim `/authenticate`-Request    | Client → Server |
| Speichern  | `auth_code → code_challenge`                     | Server (Cache) |
| Einlösen   | `code_verifier` beim `/authorize`-Request        | Client → Server |
| Prüfen     | SHA256(verifier) == gespeicherte challenge        | Server intern  |

---

### JWT Claims

| Claim   | Typ    | Beschreibung                          |
|---------|--------|---------------------------------------|
| `sub`   | String | Subject (client_id)                   |
| `scope` | String | Gewährte Berechtigungen (leerzeichen-getrennt) |
| `iat`   | u64    | Ausstellungszeitpunkt (Unix-Timestamp)|
| `exp`   | u64    | Ablaufzeitpunkt (Unix-Timestamp)      |

---

### Grant Types

| Grant Type           | PKCE erforderlich | Beschreibung                                  |
|----------------------|-------------------|-----------------------------------------------|
| `authorization_code` | ✅ Ja             | Standard-Flow für Apps                        |
| `client_credentials` | ❌ Nein           | Machine-to-Machine, kein Benutzer involviert  |
| `refresh_token`      | ❌ Nein           | Erneuerung eines abgelaufenen Access Tokens   |

---

### Scopes

| Scope            | Beschreibung                  |
|------------------|-------------------------------|
| `read`           | Lesezugriff auf Ressourcen    |
| `write`          | Schreibzugriff auf Ressourcen |
| `offline_access` | Refresh Token wird ausgestellt|
