import rsbridge
import out.pylib.backend_pb2 as pb_backend;
import out.pylib.cards_pb2 as pb_cards;

result = rsbridge.add(2, 3)
print(result)

req = pb_backend.BackendInit()
req.server = True

bkend = rsbridge.open_backend(req.SerializeToString())

rst = bkend.command(0, 0, b"")

card = pb_cards.Card.FromString(rst)
print(card)