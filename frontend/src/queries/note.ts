import { useQueryClient } from "react-query";
import z from "zod";

import { JsonMutationRet, useJsonQuery, useJsonMutation } from ".";
import { CreatePost, IdResponse, Post } from "../dto";

const NOTES_KEY = ["notes"];

export function useNotes() {
  return useJsonQuery(z.array(Post), NOTES_KEY, "/api/post");
}

export function usePostNoteMutation(
  onSuccess: () => void,
): JsonMutationRet<z.infer<typeof CreatePost>, typeof IdResponse> {
  const queryClient = useQueryClient();
  return useJsonMutation("POST", "/api/post", IdResponse, {
    onSuccess: () => {
      queryClient.invalidateQueries(NOTES_KEY);
      onSuccess();
    },
  });
}
