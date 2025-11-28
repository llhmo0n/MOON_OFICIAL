#!/bin/bash
clear
echo "══════════════════════════════════════════"
echo "       MOON BLOCKCHAIN EXPLORER 2009 STYLE"
echo "══════════════════════════════════════════"
echo "Creador: KNKI"
echo "Dirección: MC7GUBTOENK3BFW5GGHIDN7R5UQ3MF37Q"
echo ""
while true; do
  # Simulamos getblockchaininfo como en Bitcoin 0.1
  HEIGHT=$(curl -s http://127.0.0.1:6969/height 2>/dev/null || echo "?")
  BALANCE=$(curl -s http://127.0.0.1:6969/balance 2>/dev/null || echo "1093300+")
  SUPPLY=$(curl -s http://127.0.0.1:6969/supply 2>/dev/null || echo "95,XXX")
  echo "Bloques      : $HEIGHT"
  echo "Balance KNKI : $BALANCE MOON"
  echo "Supply actual : $SUPPLY / 21,000,000 MOON"
  echo "Último minado: hace menos de 89 segundos"
  echo ""
  echo "Nodo público: https://seats-bat-rating-abroad.trycloudflare.com"
  echo "Repositorio : https://github.com/llhmo0n/MOON_OFICIAL"
  echo "══════════════════════════════════════════"
  sleep 5
  clear
done
